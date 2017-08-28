const { getDistsAndVersions } = require('./get-installed')

module.exports = async function list({ dest }) {
  const dist_versions = await getDistsAndVersions(dest)

  for (let dist of Object.keys(dist_versions)) {
    console.log(`${dist}:`)
    const versions = dist_versions[dist]
    for (let version of versions) {
      console.log(`  ${version}`)
    }
  }
}
