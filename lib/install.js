const downTar = require('download-tarball')
const request = require('got')
const path = require('path')
const os = require('os')

const alias = require('./alias')
const resolve = require('./resolve')
const resolve_range = require('./resolve-range')

const dists = {
  nodejs: 'node',
  iojs: 'iojs'
}
const dist_urls = {
  nodejs: 'https://nodejs.org/dist',
  iojs: 'https://iojs.org/dist',
}

module.exports = async function install(opts) {
  let { dest, range, runtime, version } = opts
  const base_name = dists[runtime]
  const base_url = dist_urls[runtime]

  if (opts.range) {
    version = await resolve_range(runtime, range)
  }

  const dist = os.platform()
  const arch = os.arch()

  const name = `${base_name}-v${version}-${dist}-${arch}`
  const url = `${base_url}/v${version}/${name}.tar.gz`

  const dir = resolve(dest, `${runtime}/${version}`)

  await downTar({ url, dir, extractOpts: { strip: 1 } })

  console.log(`Installed ${runtime}@${version}`)

  if (opts.default) {
    await alias({
      dest,
      name: 'default',
      runtime,
      version
    })
  }
}
