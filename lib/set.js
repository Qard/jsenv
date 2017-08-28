const findUp = require('find-up')
const readPkg = require('read-pkg')
const writePkg = require('write-pkg')
const loadJsonFile = require('load-json-file')
const writeJsonFile = require('write-json-file')

module.exports = async function set(dest, key, value) {
  const data = typeof key !== 'object'
    ? { [key]: value }
    : key

  const rc_path = await findUp([
    '.jsenvrc',
    '.jsenvrc.json'
  ])

  if (rc_path) {
    const rc_file = await loadJsonFile(rc_path)

    Object.assign(rc_file, data)

    await writeJsonFile(rc_path, rc_file)
  } else {
    const pkg_path = await findUp('package.json')
    const pkg = await readPkg(pkg_path)

    if (!pkg.jsenv) pkg.jsenv = {}
    Object.assign(pkg.jsenv, data)

    await writePkg(pkg_path, pkg)
  }
}
