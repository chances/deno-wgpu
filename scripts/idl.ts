import { exists, writeFileStr } from 'https://deno.land/std/fs/mod.ts'
import * as path from 'https://deno.land/std/path/mod.ts'

import * as idl2rust from './lib/idl2rust.ts'

const gpuWebPath = 'third_party/gpuweb/spec'

export async function genBindings() {
  const status = await makeWebGpuIdl();
  if (status !== true) {
    console.error('Failed to generate webgpu.idl')
    Deno.exit(status || 1)
  }

  const idlFilePath = path.join(gpuWebPath, 'spec', 'webgpu.idl')
  const idlFileExists = await exists(idlFilePath)
  if (!idlFileExists) {
    console.error(`Failed to find WebGPU IDL at "${idl2rust}"`)
    Deno.exit(1)
  }

  const idl = await idl2rust.parse(idlFilePath)

  // console.log(JSON.stringify(idl.enums.find(dict => dict.name === 'GPUPowerPreference'), null, 2))
  // console.log(JSON.stringify(idl.dictionaries.find(dict => dict.name === 'GPURequestAdapterOptions'), null, 2))
  // console.log(JSON.stringify(idl.interfaces.find(i => i.name === 'GPU'), null, 2))

  // Emit enums
  // Use from string impls like: string.parse::<Options>()
  const enums = idl.enums.map(_enum => {
    const variants = _enum.values.map(variant => `${idl2rust.indent(idl2rust.enumVariant(variant))},\n`).join('')
    return `enum ${_enum.name} {\n${variants}}\n\n${idl2rust.fromStrImpl(_enum)}`
  }).join('\n\n')

  const enumsFilePath = path.join('src', 'enums.rs')
  await writeFileStr(enumsFilePath, `use std::str::FromStr;\n\n${enums}\n`)
  console.log(`Generated ${enumsFilePath}`)

  // Emit method params
  const methodParams = idl.interfaces.map(_interface => {
    const interfaceComment = `// ${_interface.name}`
    const methodParams = _interface.methods
      .filter(idl2rust.methodWithParams)
      .map(method => `${idl2rust.methodParams(_interface, method)}\n\n`).join('')

    const noMethodsComment = methodParams.length === 0 ? ' has no methods\n' : ''
    return `${interfaceComment}${noMethodsComment}\n${methodParams}`
  }).join('')

  const paramsFilePath = path.join('src', 'params.rs')
  await writeFileStr(paramsFilePath, `use serde::Deserialize;\n\n${methodParams}`)
  console.log(`Generated ${paramsFilePath}`)
}

async function makeWebGpuIdl(): Promise<boolean | number | undefined> {
  const process = Deno.run({
    cmd: ['make', 'webgpu.idl'],
    cwd: path.join(gpuWebPath, 'spec'),
  })

  const status = await process.status();
  if (!status.success) {
    return status.code
  }

  return true
}

if (import.meta.main) {
  await genBindings()
}
