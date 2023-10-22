/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Disposable, ThemeColor} from 'vscode';
import {StaticFeature, RequestType, FeatureState} from 'vscode-languageclient';
import {RelayExtensionContext} from './context';

// the following type definitions are one to one mappings of the types defined
// by the lsp_types package in this rust crate.
// https://github.com/gluon-lang/lsp-types/blob/master/src/window.rs#L15
enum ShowStatusMessageType {
  /// An error message.
  Error = 1,
  /// A warning message.
  Warning = 2,
  /// An information message.
  Info = 3,
  /// A log message.
  Log = 4,
}

type ShowStatusProgress = {
  numerator: number;
  denominator?: number;
};

type ShowStatusMessageActionItem = {
  title: string;
  properties: Record<
    string,
    string | boolean | number | Record<string, unknown>
  >;
};

export type ShowStatusParams = {
  type: ShowStatusMessageType;
  progress?: ShowStatusProgress;
  uri?: string;
  message?: string;
  shortMessage?: string;
  actions?: ShowStatusMessageActionItem[];
};

function getStatusBarText(params: ShowStatusParams): string | undefined {
  if (params.shortMessage) {
    return params.shortMessage;
  }

  if (params.message) {
    if (params.message.length > 16) {
      return `${params.message.slice(0, 15)}...`;
    }
    return params.message;
  }

  return undefined;
}

function getStatusBarTooltip(params: ShowStatusParams): string | undefined {
  return params.message;
}

// All possible icons can be found here https://code.visualstudio.com/api/references/icons-in-labels#icon-listing
function getStatusBarIcon(params: ShowStatusParams): string {
  if (params.type === ShowStatusMessageType.Log) {
    return 'info';
  }

  if (params.type === ShowStatusMessageType.Info) {
    return 'run';
  }

  if (params.type === ShowStatusMessageType.Error) {
    return 'error';
  }

  if (params.type === ShowStatusMessageType.Warning) {
    return 'warning';
  }

  return 'extensions-info-message';
}

function getStatusBarBackgroundColor(params: ShowStatusParams): ThemeColor {
  switch (params.type) {
    case ShowStatusMessageType.Error:
      return new ThemeColor('statusBarItem.errorBackground');
    default:
      return new ThemeColor('statusBar.background');
  }
}

function getStatusBarColor(params: ShowStatusParams): ThemeColor {
  switch (params.type) {
    case ShowStatusMessageType.Error:
      return new ThemeColor('statusBarItem.errorForeground');
    default:
      return new ThemeColor('statusBar.foreground');
  }
}

// A lot of the data from the window/showStatus command is ignored.
// On the LSP Server, we only make use of the following properties
//
// - type
// - message
// - shortMessage
//
// The source of truth is currently marked here
// https://github.com/facebook/relay/blob/main/compiler/crates/relay-lsp/src/status_updater.rs#L82
export function handleShowStatusMethod(
  context: RelayExtensionContext,
  params: ShowStatusParams,
): void {
  const icon = getStatusBarIcon(params);
  const text = getStatusBarText(params);
  const tooltipText = getStatusBarTooltip(params);
  const backgroundColor = getStatusBarBackgroundColor(params);
  const color = getStatusBarColor(params);

  if (text) {
    const textWithIcon = `$(${icon}) ${text}`;

    context.statusBar.backgroundColor = backgroundColor;
    context.statusBar.color = color;
    context.statusBar.text = textWithIcon;
    context.statusBar.tooltip = tooltipText;

    context.statusBar.show();
  }
}

// This StaticFeature is solely responsible for intercepting
// window/showStatus commands from the LSP Server and displaying
// those messages on the client status bar.
//
// The StatusBarItem creation does not happen here since we may
// want to use the status bar to display messages before we
// get messages from the LSP server.
// e.g. Looking for Relay binary...
export class LSPStatusBarFeature implements StaticFeature {
  private context: RelayExtensionContext;

  private disposable: Disposable | undefined;

  constructor(context: RelayExtensionContext) {
    this.context = context;
  }

  // eslint-disable-next-line class-methods-use-this
  fillClientCapabilities(): void {}

  initialize(): void {
    this.disposable = this.context.client?.onRequest(
      new RequestType<ShowStatusParams, void, void>('window/showStatus'),
      params => {
        handleShowStatusMethod(this.context, params);
      },
    );
  }

  // eslint-disable-next-line class-methods-use-this
  getState(): FeatureState {
    return {
      kind: 'static',
    };
  }

  clear(): void {
    this.disposable?.dispose();
  }
}
