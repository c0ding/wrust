(function () {
  var Wrust = (function (name) {
    var root = typeof window !== 'undefined' ? window : global;
    var had = Object.prototype.hasOwnProperty.call(root, name);
    var prev = root[name];
    var me = root[name] = {};

    if (typeof module !== 'undefined' && module.exports)
      module.exports = me;

    me.noConflict = function () {
      if (root[name] === me) {
        root[name] = had ? prev : undefined;

        if (!had) {
          try { delete root[name]; }
          catch (ex) {}
        }
      }

      return me;
    };

    return me;
  }('Wrust'));

  Wrust.connection = null;

  Wrust.log = function(message) {
    console.log(message);
  }

  Wrust.closeConnection = function() {
    Wrust.connection.close();
    Wrust.connection = null;
  }

  Wrust.disconnect = function() {
    if (Wrust.connection != null) {
      Wrust.closeConnection();
      Wrust.log('Disconnected.')
    }
  }

  Wrust.connect = function() {
    Wrust.disconnect();

    var wsUri = ( window.location.protocol == 'https:' && 'wss://' || 'ws://' ) + window.location.host + '/ws/';

    Wrust.connection = new WebSocket(wsUri);

    Wrust.log('Connecting...');

    Wrust.connection.onopen = function() {
      Wrust.log('Connected.');
    };

    Wrust.connection.onmessage = function(event) {
      Wrust.log('Received: ' + event.data);
    };

    Wrust.connection.onclose = function() {
      Wrust.log('Disconnected.');
      Wrust.connection = null;
    };
  };
}());

Wrust.connect();
