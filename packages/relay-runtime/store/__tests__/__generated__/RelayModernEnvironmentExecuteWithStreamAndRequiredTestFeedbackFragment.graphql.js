/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 * @oncall relay
 *
 * @generated SignedSource<<0a91e6c829fcf617290cb91a21e26316>>
 * @flow
 * @lightSyntaxTransform
 * @nogrep
 */

/* eslint-disable */

'use strict';

/*::
import type { Fragment, ReaderFragment } from 'relay-runtime';
import type { FragmentType } from "relay-runtime";
declare export opaque type RelayModernEnvironmentExecuteWithStreamAndRequiredTestFeedbackFragment$fragmentType: FragmentType;
export type RelayModernEnvironmentExecuteWithStreamAndRequiredTestFeedbackFragment$data = ?{|
  +actors: $ReadOnlyArray<?{|
    +name: ?string,
  |}>,
  +id: string,
  +$fragmentType: RelayModernEnvironmentExecuteWithStreamAndRequiredTestFeedbackFragment$fragmentType,
|};
export type RelayModernEnvironmentExecuteWithStreamAndRequiredTestFeedbackFragment$key = {
  +$data?: RelayModernEnvironmentExecuteWithStreamAndRequiredTestFeedbackFragment$data,
  +$fragmentSpreads: RelayModernEnvironmentExecuteWithStreamAndRequiredTestFeedbackFragment$fragmentType,
  ...
};
*/

var node/*: ReaderFragment*/ = {
  "argumentDefinitions": [
    {
      "kind": "RootArgument",
      "name": "enableStream"
    }
  ],
  "kind": "Fragment",
  "metadata": null,
  "name": "RelayModernEnvironmentExecuteWithStreamAndRequiredTestFeedbackFragment",
  "selections": [
    {
      "alias": null,
      "args": null,
      "kind": "ScalarField",
      "name": "id",
      "storageKey": null
    },
    {
      "kind": "Stream",
      "selections": [
        {
          "kind": "RequiredField",
          "field": {
            "alias": null,
            "args": null,
            "concreteType": null,
            "kind": "LinkedField",
            "name": "actors",
            "plural": true,
            "selections": [
              {
                "alias": null,
                "args": null,
                "kind": "ScalarField",
                "name": "name",
                "storageKey": null
              }
            ],
            "storageKey": null
          },
          "action": "LOG",
          "path": "actors"
        }
      ]
    }
  ],
  "type": "Feedback",
  "abstractKey": null
};

if (__DEV__) {
  (node/*: any*/).hash = "5dc2d29a99920e03c33cbc48484c8f04";
}

module.exports = ((node/*: any*/)/*: Fragment<
  RelayModernEnvironmentExecuteWithStreamAndRequiredTestFeedbackFragment$fragmentType,
  RelayModernEnvironmentExecuteWithStreamAndRequiredTestFeedbackFragment$data,
>*/);
