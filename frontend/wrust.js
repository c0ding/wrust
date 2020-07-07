(function () {
  var Wrust = (function (name) {
    var root = typeof window !== 'undefined' ? window : global;
    var had = Object.prototype.hasOwnProperty.call(root, name);
    var prev = root[name];
    var me = root[name] = {};

    if (typeof module !== 'undefined' && module.exports) { module.exports = me; }

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
  Wrust.connected = false;
  Wrust.elementConnectButton = null;
  Wrust.elementBoard = null;
  Wrust.elementMessage = null;
  Wrust.elementSend = null;

  Wrust.config = function(params) {
    Wrust.elementConnectButton = document.getElementById(params.connectButton);
    Wrust.elementBoard = document.getElementById(params.board);
    Wrust.elementMessage = document.getElementById(params.message);
    Wrust.elementSend = document.getElementById(params.send);
    return this;
  }

  Wrust.log = function(message) {
    Wrust.elementBoard.innerHTML = Wrust.elementBoard.innerHTML + "<br/>" + message;
    console.log(message);
  }

  Wrust.bind = function() {
    Wrust.elementConnectButton.onclick = function(event) {
      event.preventDefault();
      event.stopPropagation();

      if (Wrust.connected) { Wrust.disconnect(); }
      else { Wrust.connect(); }
    }

    Wrust.elementSend.onclick = function(event) {
      Wrust.connection.send(Wrust.elementMessage.value);
      Wrust.elementMessage.value = "";
      Wrust.elementMessage.focus();
    }

    Wrust.elementMessage.onkeyup = function(event) {
      if (event.keyCode == 13) {
        Wrust.elementSend.click();
        return false;
      }
    }
  }

  Wrust.disconnect = function() {
    if (Wrust.connection != null) {
      Wrust.log('Disconnecting...');
      Wrust.connection.close();
      Wrust.connection = null;
      Wrust.connected = false;
      Wrust.elementConnectButton.innerHTML = "Connect";
      Wrust.elementBoard.innerHTML = "";
      Wrust.elementMessage.setAttribute('disabled', true);
      Wrust.elementSend.setAttribute('disabled', true);
      Wrust.log('Disconnected');
    }
  }

  Wrust.connect = function() {
    var wsUri = ( window.location.protocol == 'https:' && 'wss://' || 'ws://' ) + window.location.host + '/ws/';

    Wrust.connection = new WebSocket(wsUri);

    Wrust.log('Connecting...');

    Wrust.connection.onopen = function() {
      Wrust.connected = true
      Wrust.elementConnectButton.innerHTML = "Disconnect"
      Wrust.elementMessage.removeAttribute('disabled');
      Wrust.elementSend.removeAttribute('disabled');
      Wrust.log('Connected');
    };

    Wrust.connection.onmessage = function(event) {
      Wrust.log('Received: ' + event.data);
    };

    Wrust.connection.onclose = function() {
      Wrust.disconnect();
    };
  };
}());
