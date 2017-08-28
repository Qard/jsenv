const path = require('path')

module.exports = function resolve(dest, fragment) {
  return dest === 'global'
    ? path.resolve(`${process.env.HOME}/.jsenv/${fragment}`)
    : path.resolve(`./.jsenv/${fragment}`)
}
