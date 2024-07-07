local M = {}

function M.random(a, b)
  if not a then a, b = 0, 1 end
  if not b then b = 0 end
  return a + math.random() * (b - a)
end

function M.rand_color()
  return {M.random(255), M.random(255), M.random(255)}
end

return M

