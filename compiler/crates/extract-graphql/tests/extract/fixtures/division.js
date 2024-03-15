/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

const math = 5 / 2;

function MyComponent() {
    useFragment(graphql`
      fragment Test on User {
        __typename
      }
    `, user)
    return <div>Test</div>;
  }
