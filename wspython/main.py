import websocket
import json

from arb import check_arbitrage_opportunity

# On Message
prices = {}
def on_message(ws, message):
  global prices
  decoded_message = json.loads(message)
  trading_pair = decoded_message['s'].lower()
  best_ask_price = float(decoded_message['a'])
  best_bid_price = float(decoded_message['b'])

  prices[trading_pair] = (best_ask_price, best_bid_price)

  if len(prices) == 3:
    if has_opportunity := check_arbitrage_opportunity(prices):
      print(f"Opportunity found: {prices}")

# On Error
def on_error(ws, error):
  print(error)

# On Close
def on_close(ws, code, reason):
  print("### closed ###")

# On Open
def on_open(ws):
  ws.send(json.dumps({"method": "SUBSCRIBE", "params": ["btcusdt@bookTicker", "ethbtc@bookTicker", "ethusdt@bookTicker"], "id": 1}))

# ENTRYPOINT
if __name__ == "__main__":
  # websocket.enableTrace(True)
  ws = websocket.WebSocketApp("wss://stream.binance.com:9443/ws",
                            on_message = on_message,
                            on_error = on_error,
                            on_close = on_close)
  ws.on_open = on_open
  ws.run_forever()
