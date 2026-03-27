/*---------------------------------------------------------------------------------------------
*  Copyright (c) Microsoft Corporation. All rights reserved.
*  Licensed under the MIT License. See License.txt in the project root for license information.
*--------------------------------------------------------------------------------------------*/

import * as helpers from './helpers.js';
import { Use } from './use.js';
import * as rust from '../codemodel/index.js';

/** used to generate #[derive(...)] statements */
export class Derive {
  private readonly entries = new Set<string>();

  constructor(...entries: Array<string>) {
    this.add(...entries)
  }

  /**
   * adds the specified entry to the derive macro, avoiding duplicates
   *
   * @param entrys the entries to add
   */
  add(...entries: Array<string>): void {
    for (const entry of entries) {
      this.entries.add(entry);
    }
  }

  /**
   * adds serde Deserialize and Serialize to the derive macro
   * based on the provided model flags, and also imports the
   * required types.
   * if the required flags aren't set, nothing is added.
   *
   * @param flags the flags used to determine the entries to add
   * @param use the use statement builder currently in scope
   */
  addSerdeForFlags(flags: rust.ModelFlags, use: Use): void {
    const needsDeserialize = (flags & rust.ModelFlags.SpreadHelper) === 0 && ((flags & rust.ModelFlags.Input) !== 0 || (flags & rust.ModelFlags.Output) !== 0);
    const needsSerialize = needsDeserialize || (flags & rust.ModelFlags.Input) !== 0 || flags === rust.ModelFlags.PolymorphicBase;
    if (needsDeserialize) {
      this.add('Deserialize');
      use.add('serde', 'Deserialize');
    }
    if (needsSerialize) {
      this.add('Serialize');
      use.add('serde', 'Serialize');
    }
  }

  /**
   * returns the derive macro or the empty string
   * if there's nothing to derive.
   *
   * @param indent optional indentation helper currently in scope, else defaults to no indentation
   * @returns the derive macro
   */
  text(indent?: helpers.indentation): string {
    if (this.entries.size === 0) {
      return '';
    }

    return `${indent ? indent.get() : ''}#[derive(${Array.from(this.entries).sort().join(', ')})]\n`;
  }
}
