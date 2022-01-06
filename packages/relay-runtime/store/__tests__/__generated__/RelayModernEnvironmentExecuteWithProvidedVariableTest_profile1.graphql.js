/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 * 
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 * @generated SignedSource<<a51b73ae02d91a4c7187c23288c08d0f>>
 * @flow
 * @lightSyntaxTransform
 * @nogrep
 */

/* eslint-disable */

'use strict';

/*::
import type { Fragment, ReaderFragment } from 'relay-runtime';
import type { FragmentType } from "relay-runtime";
declare export opaque type RelayModernEnvironmentExecuteWithProvidedVariableTest_profile1$fragmentType: FragmentType;
export type RelayModernEnvironmentExecuteWithProvidedVariableTest_profile1$ref = RelayModernEnvironmentExecuteWithProvidedVariableTest_profile1$fragmentType;
export type RelayModernEnvironmentExecuteWithProvidedVariableTest_profile1$data = {|
  +id: string,
  +name?: ?string,
  +username?: ?string,
  +profilePicture: ?{|
    +uri: ?string,
  |},
  +$fragmentType: RelayModernEnvironmentExecuteWithProvidedVariableTest_profile1$fragmentType,
|};
export type RelayModernEnvironmentExecuteWithProvidedVariableTest_profile1 = RelayModernEnvironmentExecuteWithProvidedVariableTest_profile1$data;
export type RelayModernEnvironmentExecuteWithProvidedVariableTest_profile1$key = {
  +$data?: RelayModernEnvironmentExecuteWithProvidedVariableTest_profile1$data,
  +$fragmentSpreads: RelayModernEnvironmentExecuteWithProvidedVariableTest_profile1$fragmentType,
  ...
};
*/

var node/*: ReaderFragment*/ = {
  "argumentDefinitions": [
    {
      "kind": "RootArgument",
      "name": "__RelayModernEnvironmentExecuteWithProvidedVariableTest_profile1__includeName"
    },
    {
      "kind": "RootArgument",
      "name": "__RelayModernEnvironmentExecuteWithProvidedVariableTest_profile1__skipUsername"
    }
  ],
  "kind": "Fragment",
  "metadata": null,
  "name": "RelayModernEnvironmentExecuteWithProvidedVariableTest_profile1",
  "selections": [
    {
      "alias": null,
      "args": null,
      "kind": "ScalarField",
      "name": "id",
      "storageKey": null
    },
    {
      "condition": "__RelayModernEnvironmentExecuteWithProvidedVariableTest_profile1__includeName",
      "kind": "Condition",
      "passingValue": true,
      "selections": [
        {
          "alias": null,
          "args": null,
          "kind": "ScalarField",
          "name": "name",
          "storageKey": null
        }
      ]
    },
    {
      "condition": "__RelayModernEnvironmentExecuteWithProvidedVariableTest_profile1__skipUsername",
      "kind": "Condition",
      "passingValue": false,
      "selections": [
        {
          "alias": null,
          "args": null,
          "kind": "ScalarField",
          "name": "username",
          "storageKey": null
        }
      ]
    },
    {
      "alias": null,
      "args": null,
      "concreteType": "Image",
      "kind": "LinkedField",
      "name": "profilePicture",
      "plural": false,
      "selections": [
        {
          "alias": null,
          "args": null,
          "kind": "ScalarField",
          "name": "uri",
          "storageKey": null
        }
      ],
      "storageKey": null
    }
  ],
  "type": "User",
  "abstractKey": null
};

if (__DEV__) {
  (node/*: any*/).hash = "5d032ff5883d76e84778a4ef0b477cfe";
}

module.exports = ((node/*: any*/)/*: Fragment<
  RelayModernEnvironmentExecuteWithProvidedVariableTest_profile1$fragmentType,
  RelayModernEnvironmentExecuteWithProvidedVariableTest_profile1$data,
>*/);
