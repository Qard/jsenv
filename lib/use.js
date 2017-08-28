const set = require('./set')

module.exports = async function use(opts) {
  await set(opts.dest, {
    runtime: opts.runtime,
    version: opts.version
  })

  console.log(`using ${opts.runtime}@${opts.version}`)
}
