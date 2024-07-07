---@diagnostic disable: undefined-global

local Object = require 'libraries.classic'

local Character = Object:extend()
function Character:new()
  self.width = 150
  self.height = 75

  self.x = 0
  self.y = screen.height - self.height

  self._BASE_VX = 400
  self._BASE_VY = 100

  self.vx = self._BASE_VX
  self.vy = -self._BASE_VY

  self.image = nil
  self.quad = nil

  self.is_moving = false
end

function Character:move(dt)
  self.x = self.x + (self.vx * dt)
  self.y = self.y + (self.vy * dt)
end

function Character:draw()
  if self.image == nil then
    love.graphics.rect(self.x, self.y, self.w, self.h)
  else
		love.graphics.draw(self.image, self.quad, self.x, self.y)
  end
end

local Side = {
  Left = {},
  Right = {},
  Top = {},
  Bottom = {}
}

function Character:is_touching_wall()
  if self.x <= 0 then
    return true, Side.Left
  elseif (self.x + self.width) >= screen.width then
    return true, Side.Right
  elseif self.y <= 0 then
    return true, Side.Top
  elseif (self.y + self.height) >= screen.height then
    return true, Side.Bottom
  else
    return false
  end
end

function Character:turn_on_wall(wall)
  if wall == Side.Left then
    self.vx = self._BASE_VX
  elseif wall == Side.Right then
    self.vx = -self._BASE_VX
  elseif wall == Side.Top then
    self.vy = self._BASE_VY
  elseif wall == Side.Bottom then
    self.vy = -self._BASE_VY
  end
end

function Character:load_image(file)
  local img = love.graphics.newImage(file)
  self.image = img
  self.quad = love.graphics.newQuad(0, 0, img:getPixelWidth(), img:getPixelHeight(), self.width, self.height)
end

local character = Character()

function love.load()
  screen = {
    width = 1024,
    height = 576
  }

  character:load_image('dvd.png')
  love.window.setMode(screen.width, screen.height)
end

function love.update(dt)
  if character.is_moving then character:move(dt) end

  local touching, wall = character:is_touching_wall()
  if touching then
    character:turn_on_wall(wall)
  end
end

function love.draw()
  character:draw()
end
