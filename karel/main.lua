SQUARE_SIZE = 50

dimensions = {
  w = 5,
  h = 5,
}

function love.load()
  love.window.setMode(SQUARE_SIZE*dimensions.w, SQUARE_SIZE*dimensions.h)
end
