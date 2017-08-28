const list = require('../lib/list')

module.exports = {
  desc: 'List installed JS engines',
  aliases: ['ls'],

  async handler(argv) {
    await list(argv)
  }
}
