const readdir = require('fs-readdir-promise')
const resolve = require('./resolve')

exports.getDists = getDists
async function getDists(dest) {
  const location = resolve(dest, '')
  return await readdir(location)
}

exports.getVersions = getVersions
async function getVersions(dest, dist) {
  const location = resolve(dest, dist)
  return await readdir(location)
}

exports.getDistsAndVersions = async function getDistsAndVersions(dest) {
  const result = {}
  const dists = await getDists(dest)
  for (let dist of dists) {
    result[dist] = await getVersions(dest, dist)
  }
  return result
}
