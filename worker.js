addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
});

/**
 * Fetch and log a request
 * @param {Request} request
 */
async function handleRequest(request) {
  let url = new URL(request.url);
  let page = pages[url.pathname.slice(1,)];

  if (page) {
    return new Response(atob(page), {
      status: 200,
      headers: {
        'content-type': 'text/html;charset=UTF-8'
      }
    });
  } else {
    return new Response('Page not found', { status: 404 })
  }
}
