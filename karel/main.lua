local api = require("api")
Karel = api.make_karel()

local dimensions = { w = 5, h = 5 }
SQUARE_SIZE = 50


function Karel:draw()
  love.graphics.rectangle("fill",
    (self.x - 1) * SQUARE_SIZE,
    (self.y - 1) * SQUARE_SIZE,
    SQUARE_SIZE, SQUARE_SIZE)
end

function love.load()
  love.window.setMode(
    SQUARE_SIZE * dimensions.w,
    SQUARE_SIZE * dimensions.h)
end

function love.draw()
  Karel:draw()
  love.graphics.print("("..Karel.x..","..Karel.y..")")
end

function love.keypressed(key)
  Karel:move_direction(key)
end
