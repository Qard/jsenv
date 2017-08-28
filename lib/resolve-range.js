const request = require('got')

const dists = {
  nodejs: 'node',
  iojs: 'iojs'
}

async function resolve_range(runtime, range) {
  const url = `https://semver.io/${dists[runtime]}/resolve/${range}`
  const { body, status } = await request(url)
  if (status >= 300) {
    throw new Error(`Unable to reach semverio to resolve version range`)
  }
  return body
}
