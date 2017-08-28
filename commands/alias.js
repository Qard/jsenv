const alias = require('../lib/alias')

module.exports = {
  command: 'alias <name> <version>',
  desc: 'Alias a JS engine',
  aliases: ['a'],

  handler(argv) {
    console.log('handle', argv)
  }
}
