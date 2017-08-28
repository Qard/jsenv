#!/usr/bin/env node

const findUp = require('find-up')
const path = require('path')
const fs = require('fs')

const configPath = findUp.sync(['.jsenvrc', '.jsenvrc.json'])
const config = configPath ? JSON.parse(fs.readFileSync(configPath)) : {}

require('yargs')
  .env('JSENV')
  .pkgConf('jsenv')
  .config(config)
  .commandDir('commands')
  .option('dest', {
    choices: ['global','local'],
    alias: 'd',
    default: 'local',
    describe: 'Set environment destination'
  })
  .option('runtime', {
    alias: 'r',
    default: 'nodejs',
    string: true,
    describe: 'Type of JS runtime'
  })
  .option('version', {
    alias: 'v',
    default: 'default',
    string: true,
    describe: 'Version of JS runtime'
  })
  .demandCommand()
  .help()
  .epilogue('for more information, find our manual at http://jsenv.io')
  .argv
