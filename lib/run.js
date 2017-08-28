const consume = require('stream-consume-promise')
const produce = require('stream-produce-promise')
const { spawn } = require('child_process')

const resolve = require('./resolve')

function fullCommand(opts) {
  const parts = opts.command.slice()
  const remainder = opts['--']
  if (remainder) {
    parts.push('--', ...remainder)
  }
  return parts
}

function commandString(parts) {
  return parts
    .map(part => part.includes(' ') ? `"${part}"` : part)
    .join(' ')
}

module.exports = async function run(opts) {
  // const [ proc, ...args ] = fullCommand(opts)
  const [ proc, ...args ] = opts['--']

  const { dest, runtime, version } = opts

  const bin_path = resolve(dest, `${runtime}/${version}/bin`)
  const env_path = `${bin_path}:${process.env.PATH}`
  const ps = spawn(proc, args, {
    env: Object.assign({}, process.env, {
      PATH: env_path
    })
  })

  ps.on('exit', (code) => {
    process.exitCode = code
  })

  const read = consume(ps.stdout)
  const write = produce(process.stdout)

  while (true) {
    const { done, value } = await read()
    if (done) break

    await write(value.toString())
  }
}
