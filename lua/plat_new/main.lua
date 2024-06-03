function love.load()
  screen = {
    width = love.graphics.getWidth(),
    height = love.graphics.getHeight(),
  }

  JUMP_VELOCITY = -400
  GRAVITY = -500

  player = {
    x = 0,
    y = screen.height - 60,
    width = 60,
    height = 60,
    vx = 0,
    vy = 300,

    is_jumping = false
  }
end

local function is_touching_wall()
  if player.x <= 0 then
    return true, "left"
  elseif player.x >= screen.width - player.width then
    return true, "right"
  elseif player.y >= screen.height then
    return true, "bottom"
  else
    return false
  end
end

function love.update(dt)
  player.x = player.x + (player.vx * dt)

  player.y = player.y + (player.vy * dt)
  player.vy = player.vy - (GRAVITY * dt)
end

function love.draw()
  love.graphics.rectangle("fill", player.x, player.y, player.width, player.height)
end

function love.keypressed(key)
  if key == "left" then
    player.vx = -300
  elseif key == "right" then
    player.vx = 300
  elseif key == "space" then
    player.vy = JUMP_VELOCITY
    player.is_jumping = false
  end
end

function love.keyreleased(key)
  if key == "left" or key == "right" then
    player.vx = 0
  end
end
