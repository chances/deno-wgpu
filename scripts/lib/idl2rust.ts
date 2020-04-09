import { readFileStr } from 'https://deno.land/std/fs/mod.ts'

import { pascalCase, snakeCase } from 'https://github.com/chances/deno-change-case/raw/deno-v0.40.0/mod.ts'
import * as webidl from 'https://cdn.pika.dev/webidl2@^23.10.1'
/// <reference types="http://cdn.pika.dev/-/webidl2@v23.11.0-SUgViYS7k79m9zpJhPKL/dist=es2017,mode=types/index.d.ts" />

export async function parse(file: string): Promise<IDLDocument> {
  const fileContents = await readFileStr(file, { encoding: 'utf8' });
  return new IDLDocument(webidl.parse(fileContents))
}

export type IDLType = 'interface'
  | 'interface mixin'
  | 'namespace'
  | 'callback'
  | 'dictionary'
  | 'enum'
  | 'typedef'
  | 'includes'
  | 'constructor'

export class IDLDocument {
  constructor(private declarations: Array<any>) {
  }

  public declarationsOfType(type: IDLType) {
    return this.declarations.filter(declaration => declaration.type === type)
  }

  public get enums(): Enum[] {
    return this.declarationsOfType('enum').map(_enum => ({
      name: _enum.name,
      values: _enum.values.map((v: any) => v.value)
    }))
  }

  public get dictionaries(): Dictionary[] {
    return this.declarationsOfType('dictionary').map(dict => ({
      name: dict.name,
      fields: dict.members.map((field: any) => ({
        name: field.name,
        type: `${field.idlType.idlType}`,
        nullable: field.idlType.nullable
      }))
    }))
  }

  public get interfaces(): Interface[] {
    return this.declarationsOfType('interface').map(_interface => ({
      name: _interface.name,
      inheritedInterface: _interface.inheritance,
      methods: _interface.members.filter((member: any) => member.type === 'operation').map((operation: any) => ({
        name: operation.name,
        returnType: operation.idlType.generic
          ? operation.idlType.idlType[0]/* First generic type arg */.idlType
          : operation.idlType.idlType,
        returnsPromise: operation.idlType.generic === 'Promise',
        arguments: operation.arguments.map((arg: any) => ({
          name: arg.name,
          type: arg.idlType.idlType,
          optional: arg.optional
        }))
      }))
    }))
  }

  public enum(name: string) {
     return this.enums.find(namedDeclaration(name))
  }

  public dictionary(name: string) {
    return this.dictionaries.find(namedDeclaration(name))
  }
}

function namedDeclaration(name: string) {
  return (named: NamedDeclaration) => named.name === name
}

export function methodWithParams(method: Operation) {
  return method.arguments.length > 0
}

// Code gen functions

export function indent(line: string, amount: number = 2) {
  return `${' '.repeat(amount)}${line}`
}

export function enumVariant(variant: string) {
  const isFirstCharNumeric = Number.isNaN(parseInt(variant.substr(0, 1), 10)) === false
  const prefix = isFirstCharNumeric ? '_' : ''
  return `${prefix}${pascalCase(variant.split('-').join(' '))}`
}

export function fromStrImpl(_enum: Enum) {
  function matchVariant(variant: string) {
    return `"${variant}" => Ok(${_enum.name}::${enumVariant(variant)})`
  }

  return `impl FromStr for ${_enum.name} {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
${_enum.values.map(matchVariant).map(v => `${indent(v, 6)},`).join('\n')}
      _ => Err(String::from("Invalid value for ${_enum.name} enum")),
    }
  }
}`
}

export function methodParams(_interface: Interface, method: Operation) {
  let methodName = method.name
  methodName = `${methodName.substr(0, 1).toUpperCase()}${methodName.substring(1)}`
  if (methodName === 'Finish') {
    methodName = `${_interface.name}${methodName}`
  }
  const params = method.arguments.map(arg => {
    let type = 'String'
    if (arg.optional) {
      type = `Option<${type}>`
    }

    const allowUnused = indent('#[allow(dead_code)]')
    const paramDecl = indent(`${snakeCase(arg.name)}: ${type}`)
    return `${allowUnused}\n${paramDecl},`
  }).join('\n')

  const attrs = '#[derive(Deserialize)]\n#[serde(rename_all = "camelCase")]'
  return `${attrs}\nstruct ${methodName}Params {\n${params}\n}`
}

interface NamedDeclaration {
  name: string
}

export interface Enum extends NamedDeclaration {
  values: string[]
}

export interface Field extends NamedDeclaration {
  typeName: string
  nullable: boolean
}

export interface Dictionary extends NamedDeclaration {
  fields: Field[]
}

export interface Interface extends NamedDeclaration {
  inheritedInterface: string | null
  // IDLInterfaceMemberType
  methods: Operation[]
}

export interface Operation extends NamedDeclaration {
  returnType: string
  returnsPromise: boolean
  arguments: Argument[]
}

export interface Argument extends NamedDeclaration {
  type: string
  optional: boolean
}
