const list_available = require('../lib/list_available')

module.exports = {
  desc: 'List available JS engines',
  aliases: ['lsa'],

  async handler(argv) {
    await list_available(argv)
  }
}
