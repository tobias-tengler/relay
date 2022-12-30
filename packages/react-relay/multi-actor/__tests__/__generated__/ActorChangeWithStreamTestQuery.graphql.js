/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 * @oncall relay
 *
 * @generated SignedSource<<6c6e3e8f65e698ec1e4e9d5c24f90f64>>
 * @flow
 * @lightSyntaxTransform
 * @nogrep
 */

/* eslint-disable */

'use strict';

/*::
import type { ConcreteRequest, Query } from 'relay-runtime';
import type { ActorChangePoint } from "react-relay/multi-actor";
import type { ActorChangeWithStreamTestFragment$fragmentType } from "./ActorChangeWithStreamTestFragment.graphql";
export type ActorChangeWithStreamTestQuery$variables = {||};
export type ActorChangeWithStreamTestQuery$data = {|
  +viewer: ?{|
    +newsFeed: ?{|
      +edges: ?$ReadOnlyArray<?{|
        +node: ?ActorChangePoint<{|
          +actor_key: string,
          +$fragmentSpreads: ActorChangeWithStreamTestFragment$fragmentType,
        |}>,
      |}>,
    |},
  |},
|};
export type ActorChangeWithStreamTestQuery = {|
  response: ActorChangeWithStreamTestQuery$data,
  variables: ActorChangeWithStreamTestQuery$variables,
|};
*/

var node/*: ConcreteRequest*/ = (function(){
var v0 = {
  "alias": null,
  "args": null,
  "kind": "ScalarField",
  "name": "__typename",
  "storageKey": null
},
v1 = {
  "alias": null,
  "args": null,
  "kind": "ScalarField",
  "name": "id",
  "storageKey": null
};
return {
  "fragment": {
    "argumentDefinitions": [],
    "kind": "Fragment",
    "metadata": null,
    "name": "ActorChangeWithStreamTestQuery",
    "selections": [
      {
        "alias": null,
        "args": null,
        "concreteType": "Viewer",
        "kind": "LinkedField",
        "name": "viewer",
        "plural": false,
        "selections": [
          {
            "alias": null,
            "args": null,
            "concreteType": "NewsFeedConnection",
            "kind": "LinkedField",
            "name": "newsFeed",
            "plural": false,
            "selections": [
              {
                "alias": null,
                "args": null,
                "concreteType": "NewsFeedEdge",
                "kind": "LinkedField",
                "name": "edges",
                "plural": true,
                "selections": [
                  {
                    "kind": "ActorChange",
                    "alias": null,
                    "name": "node",
                    "storageKey": null,
                    "args": null,
                    "fragmentSpread": {
                      "args": null,
                      "kind": "FragmentSpread",
                      "name": "ActorChangeWithStreamTestFragment"
                    }
                  }
                ],
                "storageKey": null
              }
            ],
            "storageKey": null
          }
        ],
        "storageKey": null
      }
    ],
    "type": "Query",
    "abstractKey": null
  },
  "kind": "Request",
  "operation": {
    "argumentDefinitions": [],
    "kind": "Operation",
    "name": "ActorChangeWithStreamTestQuery",
    "selections": [
      {
        "alias": null,
        "args": null,
        "concreteType": "Viewer",
        "kind": "LinkedField",
        "name": "viewer",
        "plural": false,
        "selections": [
          {
            "alias": null,
            "args": null,
            "concreteType": "NewsFeedConnection",
            "kind": "LinkedField",
            "name": "newsFeed",
            "plural": false,
            "selections": [
              {
                "alias": null,
                "args": null,
                "concreteType": "NewsFeedEdge",
                "kind": "LinkedField",
                "name": "edges",
                "plural": true,
                "selections": [
                  {
                    "kind": "ActorChange",
                    "linkedField": {
                      "alias": null,
                      "args": null,
                      "concreteType": null,
                      "kind": "LinkedField",
                      "name": "node",
                      "plural": false,
                      "selections": [
                        (v0/*: any*/),
                        {
                          "kind": "TypeDiscriminator",
                          "abstractKey": "__isFeedUnit"
                        },
                        (v1/*: any*/),
                        {
                          "alias": null,
                          "args": null,
                          "concreteType": "Text",
                          "kind": "LinkedField",
                          "name": "message",
                          "plural": false,
                          "selections": [
                            {
                              "alias": null,
                              "args": null,
                              "kind": "ScalarField",
                              "name": "text",
                              "storageKey": null
                            }
                          ],
                          "storageKey": null
                        },
                        {
                          "alias": null,
                          "args": null,
                          "concreteType": "Feedback",
                          "kind": "LinkedField",
                          "name": "feedback",
                          "plural": false,
                          "selections": [
                            (v1/*: any*/),
                            {
                              "if": null,
                              "kind": "Stream",
                              "label": "ActorChangeWithStreamTestFragment$stream$actors",
                              "selections": [
                                {
                                  "alias": null,
                                  "args": null,
                                  "concreteType": null,
                                  "kind": "LinkedField",
                                  "name": "actors",
                                  "plural": true,
                                  "selections": [
                                    (v0/*: any*/),
                                    {
                                      "alias": null,
                                      "args": null,
                                      "kind": "ScalarField",
                                      "name": "name",
                                      "storageKey": null
                                    },
                                    (v1/*: any*/)
                                  ],
                                  "storageKey": null
                                }
                              ]
                            }
                          ],
                          "storageKey": null
                        },
                        {
                          "alias": null,
                          "args": null,
                          "kind": "ScalarField",
                          "name": "actor_key",
                          "storageKey": null
                        }
                      ],
                      "storageKey": null
                    }
                  }
                ],
                "storageKey": null
              }
            ],
            "storageKey": null
          }
        ],
        "storageKey": null
      }
    ]
  },
  "params": {
    "cacheID": "af9077c40ecf696d1f3bbe7d9cad03c6",
    "id": null,
    "metadata": {},
    "name": "ActorChangeWithStreamTestQuery",
    "operationKind": "query",
    "text": "query ActorChangeWithStreamTestQuery {\n  viewer {\n    newsFeed {\n      edges {\n        node @fb_actor_change {\n          __typename\n          ...ActorChangeWithStreamTestFragment\n          actor_key\n          id\n        }\n      }\n    }\n  }\n}\n\nfragment ActorChangeWithStreamTestFragment on FeedUnit {\n  __isFeedUnit: __typename\n  id\n  message {\n    text\n  }\n  feedback {\n    id\n    actors @stream(label: \"ActorChangeWithStreamTestFragment$stream$actors\", initialCount: 1) {\n      __typename\n      name\n      id\n    }\n  }\n}\n"
  }
};
})();

if (__DEV__) {
  (node/*: any*/).hash = "d36707add9f4e27149d6e1bf38a04360";
}

module.exports = ((node/*: any*/)/*: Query<
  ActorChangeWithStreamTestQuery$variables,
  ActorChangeWithStreamTestQuery$data,
>*/);
