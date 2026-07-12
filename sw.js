'use strict'

const APP_VERSION = 'v1'
const STATIC_CACHE = `app-static-${APP_VERSION}`
const DYNAMIC_CACHE = `app-dynamic-${APP_VERSION}`
const KNOWN_CACHES = [STATIC_CACHE, DYNAMIC_CACHE]

const IMAGE_DB_NAME = 'dioxus_app_images_db'
const IMAGE_DB_VERSION = 1
const IMAGE_STORE = 'images'
const IMAGE_API_BASE = '/images'

const PRECACHE_URLS = ['/', '/index.html']

function requestPromise (request) {
  return new Promise((resolve, reject) => {
    request.onsuccess = () => resolve(request.result)
    request.onerror = () => reject(request.error)
  })
}

function waitForTransaction (tx) {
  return new Promise((resolve, reject) => {
    tx.oncomplete = () => resolve()
    tx.onerror = () => reject(tx.error)
    tx.onabort = () => reject(tx.error)
  })
}

function openImageDb () {
  return new Promise((resolve, reject) => {
    const request = indexedDB.open(IMAGE_DB_NAME, IMAGE_DB_VERSION)

    request.onupgradeneeded = event => {
      const db = event.target.result
      if (!db.objectStoreNames.contains(IMAGE_STORE)) {
        db.createObjectStore(IMAGE_STORE, { keyPath: 'id' })
      }
    }

    request.onsuccess = () => resolve(request.result)
    request.onerror = () => reject(request.error)
  })
}

async function saveImageRecord (id, blob, contentType) {
  const db = await openImageDb()
  const tx = db.transaction([IMAGE_STORE], 'readwrite')
  const store = tx.objectStore(IMAGE_STORE)
  store.put({ id, blob, contentType, createdAt: new Date().toISOString() })
  await waitForTransaction(tx)
  db.close()
}

async function loadImageRecord (id) {
  const db = await openImageDb()
  const tx = db.transaction([IMAGE_STORE], 'readonly')
  const store = tx.objectStore(IMAGE_STORE)
  const request = store.get(id)
  const result = await requestPromise(request)
  await waitForTransaction(tx)
  db.close()
  return result
}

function parseImageIdFromPath (path) {
  const prefix = `${IMAGE_API_BASE}/`
  if (!path.startsWith(prefix)) return null
  const id = path.slice(prefix.length)
  return /^[0-9a-fA-F-]{36}$/.test(id) ? id : null
}

async function handleUploadImage (request) {
  try {
    const blob = await request.blob()
    const id = crypto.randomUUID()
    const contentType = blob.type || 'application/octet-stream'
    await saveImageRecord(id, blob, contentType)
    return new Response(
      JSON.stringify({ id, url: `${IMAGE_API_BASE}/${id}` }),
      {
        headers: { 'Content-Type': 'application/json' }
      }
    )
  } catch (error) {
    console.error('[SW] Image upload failed', error)
    return new Response('Image upload failed', { status: 500 })
  }
}

async function handleGetImage (id) {
  try {
    const record = await loadImageRecord(id)
    if (!record) {
      return new Response('Image not found', { status: 404 })
    }
    return new Response(record.blob, {
      status: 200,
      headers: { 'Content-Type': record.contentType }
    })
  } catch (error) {
    console.error('[SW] Image load failed', error)
    return new Response('Image load error', { status: 500 })
  }
}

self.addEventListener('install', event => {
  event.waitUntil(
    caches
      .open(STATIC_CACHE)
      .then(cache => cache.addAll(PRECACHE_URLS))
      .then(() => self.skipWaiting())
  )
})

self.addEventListener('activate', event => {
  event.waitUntil(
    caches
      .keys()
      .then(names =>
        Promise.all(
          names
            .filter(n => !KNOWN_CACHES.includes(n))
            .map(n => {
              console.log(`[SW] Deleting stale cache: ${n}`)
              return caches.delete(n)
            })
        )
      )
      .then(() => self.clients.claim())
  )
})

self.addEventListener('fetch', event => {
  console.log('[SW]', event.request.method, event.request.url)
  const { request } = event
  const url = new URL(request.url)

  if (!request.url.startsWith(self.location.origin)) return

  const imageId = parseImageIdFromPath(url.pathname)
  if (request.method === 'POST' && url.pathname === IMAGE_API_BASE) {
    event.respondWith(handleUploadImage(request))
    return
  }

  if (request.method === 'GET' && imageId) {
    event.respondWith(handleGetImage(imageId))
    return
  }

  if (request.method !== 'GET') return

  if (isStaticAsset(url)) {
    event.respondWith(cacheFirst(request, STATIC_CACHE))
  } else if (request.mode === 'navigate') {
    event.respondWith(networkFirst(request))
  } else {
    event.respondWith(staleWhileRevalidate(request, DYNAMIC_CACHE))
  }
})

/** True for assets whose filenames are content-hashed by Trunk. */
function isStaticAsset (url) {
  const p = url.pathname
  return [
    '.wasm',
    '.js',
    '.css',
    '.png',
    '.jpg',
    '.jpeg',
    '.svg',
    '.ico',
    '.woff',
    '.woff2',
    '.ttf'
  ].some(ext => p.endsWith(ext))
}

/**
 * Cache-first: return the cached response immediately.
 * On a cache miss, fetch from the network, store, then return.
 */
async function cacheFirst (request, cacheName) {
  const cache = await caches.open(cacheName)
  const cached = await cache.match(request)
  if (cached) return cached

  try {
    const response = await fetch(request)
    if (response.ok) cache.put(request, response.clone())
    return response
  } catch (err) {
    console.warn('[SW] cache-first miss + network failure', request.url, err)
    return new Response('Asset unavailable offline', {
      status: 503,
      statusText: 'Service Unavailable'
    })
  }
}

/**
 * Network-first: try the network; on failure serve from cache.
 * For navigation, falls back to the /index.html SPA shell so that
 * client-side routing (Dioxus Router) works fully offline.
 */
async function networkFirst (request) {
  const cache = await caches.open(DYNAMIC_CACHE)

  try {
    const response = await fetch(request)
    if (response.ok) cache.put(request, response.clone())
    return response
  } catch {
    const cached = await cache.match(request)
    if (cached) return cached

    const shell = await caches.match('/index.html')
    return (
      shell ??
      new Response('<h1>Offline</h1>', {
        status: 503,
        headers: { 'Content-Type': 'text/html' }
      })
    )
  }
}

/**
 * Stale-while-revalidate: return from cache immediately (if available)
 * and update the cache entry from the network in the background.
 */
async function staleWhileRevalidate (request, cacheName) {
  const cache = await caches.open(cacheName)

  const networkUpdate = fetch(request)
    .then(response => {
      if (response.ok) cache.put(request, response.clone())
      return response
    })
    .catch(() => null)

  const cached = await cache.match(request)
  return (
    cached ??
    (await networkUpdate) ??
    new Response('Unavailable', { status: 503 })
  )
}

self.addEventListener('message', event => {
  if (event.data?.type === 'SKIP_WAITING') {
    console.log('[SW] Received SKIP_WAITING — activating immediately.')
    self.skipWaiting()
  }
})
