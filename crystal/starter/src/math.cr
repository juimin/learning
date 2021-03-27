# Int32 is a 32-bit integer type, Float64 is a 64-bit floating point number.

p! 1, typeof(1)
p! 1.0, typeof(1.0)
p! 100_000, typeof(100_000)
p! 100_000.0, typeof(100_000.0)

puts "comparison"
p! 1 == 1,
   1 == 2,
   1.0 == 1,
   # Note that these are equal regardless of numerical type
   -2000.0 == -2000


puts "spaceship operator"
p! 1 <=> 1,
2 <=> 1,
1 <=> 2

p! 1 + 1,  # addition
   1 - 1,  # subtraction
   2 * 3,  # multiplication
   2 ** 4, # exponentiation
   2 / 3,  # division
   2 // 3, # floor division
   3 % 2,  # modulus
   -1      # negation (unary)


message = "Hello World! Greetings from Crystal."

puts "normal: #{message}"
puts "upcased: #{message.upcase}"
puts "downcased: #{message.downcase}"
puts "camelcased: #{message.camelcase}"
puts "capitalized: #{message.capitalize}"
puts "reversed: #{message.reverse}"
puts "titleized: #{message.titleize}"
puts "underscored: #{message.underscore}"

message = "Hel"

p! message[2, 1000]
   