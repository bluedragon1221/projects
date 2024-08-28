game = {
  WIDTH = love.graphics.getWidth(),
  HEIGHT = love.graphics.getHeight(),

  JUMP_VY = -20,
  MOVE_FRICTION = 2.5,
  GRAVITY = 1.5,
}

-- BLOCK

block = {
  x = 50,
  y = 50,
  vy = 10,
  w = 50,
  h = 50,

  is_jumping = false,
}

function block:draw()
  love.graphics.rectangle("fill", self.x, self.y, self.w, self.h)
end

function block:isTouchingWall(wall)
  if wall == "bottom" then
    if (self.y > (game.HEIGHT - self.h)) then
      return true, game.HEIGHT - self.h
    elseif self.y + self.h > obstacle.y and
           self.x + self.w > obstacle.x and
           self.x < obstacle.x + obstacle.w then
      return true, obstacle.y - self.h
    end
  elseif wall == "top" and self.y < 0 then
    return true
  elseif wall == "left" then
    if self.x < 0 then
      return true, 0
    elseif self.x < obstacle.x + obstacle.w and -- right
           self.x > obstacle.x + obstacle.w / 2 and
           self.y + self.h > obstacle.y + 20 then -- top
      return true, obstacle.x + obstacle.w
    end
  elseif wall == "right" then
    if self.x > (game.WIDTH - self.w) then
      return true, game.WIDTH - self.w
    elseif self.x + self.w > obstacle.x and -- left
           self.x < obstacle.x + obstacle.w / 2 and -- clip right
           self.y + self.h > obstacle.y + 20 then -- top
      return true, obstacle.x - self.w
    end
  end

  return false
end

function block:applyGravity()
  self.y = self.y + self.vy

  touching_r, xval_r = block:isTouchingWall("left")
  if touching_r then self.x = xval_r end

  touching_l, xval_l = block:isTouchingWall("right")
  if touching_l then self.x = xval_l end

  touching_bottom, yval = block:isTouchingWall("bottom")
  if touching_bottom then
    self.y = yval

    if self.is_jumping then
      self.is_jumping = false
    else -- not jumping
      self.vy = 0
    end
  else
    -- speed up fall if block isn't on ground
    self.vy = self.vy + game.GRAVITY
  end
end

function block:do_jump()
  self.is_jumping = true
  self.vy = game.JUMP_VY
end

-- END BLOCK

-- SPIKE
obstacle = {
  x = 500,
  y = game.HEIGHT - 60,
  h = 60,
  w = 60
}

function obstacle:draw()
  love.graphics.rectangle("fill", self.x, self.y, self.w, self.h)
  -- love.graphics.polygon("fill", self.x, self.y + self.h, self.x + self.w, self.y + self.h, self.x + (self.w / 2), self.y)
end

-- END SPIKE

function love.update()
  block:applyGravity()

  if love.keyboard.isDown("left") and not love.keyboard.isDown("right") and not block:isTouchingWall("left") then
    block.x = block.x - 7.5
  elseif love.keyboard.isDown("right") and not love.keyboard.isDown("left") and not block:isTouchingWall("right") then
    block.x = block.x + 7.5
  end
end

function love.draw()
  block:draw()
  obstacle:draw()
  love.graphics.print("("..block.x..", "..block.y..")", 0, 0)
  if block:isTouchingWall("right") then
    love.graphics.print("right", 0, 20)
  elseif block:isTouchingWall("left") then
    love.graphics.print("left", 0, 20)
  end
end

function love.keypressed(key, scancode, isRepeat)
  if key == "space" and not block.is_jumping then
    block:do_jump()
  end
end
