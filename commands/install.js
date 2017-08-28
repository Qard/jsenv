const install = require('../lib/install')

module.exports = {
  command: 'install <range>',
  desc: 'Install a JS engine',
  aliases: ['i'],

  builder(yargs) {
    yargs.option('default', {
      boolean: true,
      describe: 'Alias as default JS engine'
    })
  },

  async handler(argv) {
    await install(argv)
  }
}
