const request = require('got')

const dists = {
  nodejs: 'node',
  iojs: 'iojs'
}

module.exports = async function list_available(opts) {
  const url = `https://semver.io/${dists[opts.runtime]}.json`
  const { body, status } = await request(url, { json: true })

  if (status >= 300) {
    console.log('error', body)
    process.exitCode = 1
    return
  }

  for (let version of body.all) {
    console.log('  %s %s', opts.runtime, version)
  }
}
