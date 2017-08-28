const run = require('../lib/run')

module.exports = {
  // command: 'run <command..>',
  desc: 'Run in selected JS engine',
  aliases: ['r'],

  async handler(argv) {
    await run(argv)
  }
}
