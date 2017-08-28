const use = require('../lib/use')

module.exports = {
  command: 'use <version>',
  desc: 'Use a JS engine',
  aliases: ['u'],

  async handler(argv) {
    await use(argv)
  }
}
