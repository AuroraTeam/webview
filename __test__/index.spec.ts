import test from 'ava'

import { Window } from '../index.js'

test('Window is a function', (t) => {
  t.is(typeof Window, 'function')
})

test('Get webview version', (t) => {
  const version = Window.getWebviewVersion()
  t.log('Webview version:', version)
  t.is(typeof version, 'string')
})
