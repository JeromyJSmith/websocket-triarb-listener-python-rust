# Calculates whether or not triangular arbitrage opportunity exists
def check_arbitrage_opportunity(prices):
  """
  Calculate whether a potential triangular arbitrage opportunity exists.

  Args:
  prices (dict): A dictionary with trading pairs as keys and a tuple containing
                  (best_ask_price, best_bid_price) as values.

  Returns:
  bool: True if there's a potential arbitrage opportunity, False otherwise.
  """

  # Extract prices for each trading pair
  btc_usdt_ask, btc_usdt_bid = prices['btcusdt']
  eth_btc_ask, eth_btc_bid = prices['ethbtc']
  eth_usdt_ask, eth_usdt_bid = prices['ethusdt']

  # Calculate the arbitrage opportunities for both directions
  opportunity_1 = (1 / btc_usdt_ask) * (1 / eth_btc_ask) * eth_usdt_bid
  opportunity_2 = btc_usdt_bid * eth_btc_bid * (1 / eth_usdt_ask)

  # Check if there's a potential arbitrage opportunity
  return opportunity_1 > 1.0001 or opportunity_2 > 1.0001
