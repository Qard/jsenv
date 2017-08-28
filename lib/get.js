const findUp = require('find-up')
const readPkg = require('read-pkg')
const loadJsonFile = require('load-json-file')

module.exports = async function get(key) {
  const rc_path = await findUp([
    '.jsenvrc',
    '.jsenvrc.json'
  ])

  let data
  if (rc_path) {
    data = await loadJsonFile(rc_path)
  } else {
    const pkg_path = await findUp('package.json')
    const pkg = await readPkg(pkg_path)
    data = pkg.jsenv || {}
  }

  return key ? data[key] : data
}
