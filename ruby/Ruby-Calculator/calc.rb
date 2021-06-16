# Derek Wang
# CSE 413 16AU
# HW 8 - Calculators

# Part II. Calculator $scanner

# This file calls the $scanner and prints the tokens Used

# Require the $scanner and token classes from the scan.rb file
require_relative 'scan.rb'
# readline provides nice command-line editing for text input
require 'readline'

# Test the $scanner and the token classes
$running = true
$hashmap = Hash.new
$scanner = Scanner.new

# Add PI
$hashmap["PI"] = Math::PI
$hashmap["pi"] = Math::PI
$operator = nil
$currentToken = nil

def factor
  $currentToken = $scanner.next_token()
  if ($currentToken != nil)
    case $currentToken.kind()
    when "id"
      if ($hashmap[$currentToken.value()] != nil)
        return $hashmap[$currentToken.value()]
      else
        return "notAssigned"
      end
    when "num"
      return $currentToken.value().to_f
    when "lparen"
      return expression()
    when "sqrt"
      $scanner.next_token() # skip next left paren
      number = expression()
      if (number.is_a? Numeric)
        val = Math.sqrt(number)
        return val
      else
        puts "Error: Value passed to tan() "
      end
    when "subtract"
      return -1 * expression()
    when "log"
      $scanner.next_token() # skip next left paren
      number = expression()
      if (number.is_a? Numeric)
        val = Math.log(number)
        return val
      else
        puts "Error: Value passed to tan() "
      end
    when "sin"
      $scanner.next_token() # skip next left paren
      number = expression()
      if (number.is_a? Numeric)
        val = Math.sin(number)
        return val
      else
        puts "Error: Value passed to tan() "
      end
    when "cos"
      $scanner.next_token() # skip next left paren
      number = expression()
      if (number.is_a? Numeric)
        val = Math.cos(number)
        return val
      else
        puts "Error: Value passed to tan() "
      end
    when "tan"
      $scanner.next_token() # skip next left paren
      number = expression()
      if (number.is_a? Numeric)
        val = Math.tan(number)
        return val
      else
        puts "Error: Value passed to tan() "
      end
    end
  end
end

def power
  val = factor()
    $operator = $scanner.next_token()
  if (val.is_a? Numeric)
    if ($operator.kind() == "power")
      power = power()
      return val.to_f ** power.to_f
    else
      return val
    end
  else
    return val
  end
end

def term
  val = power()
  if (val.is_a? Numeric)
    if ($operator.kind() == "multiply")
      return val * power()
    elsif ($operator.kind() == "divide")
      return val / power()
    else
      return val
    end
  else
    return val
  end
end

def expression
  val = term()
  if (val.is_a? Numeric)
    if ($operator.kind() == "add")
      return val + term()
    elsif ($operator.kind() == "subtract")
      return val - term()
    else
      return val
    end
  else
    return val
  end
end

def statement
  val = expression()
  if ($operator != nil && ($operator.kind() != "equals" && (val.is_a? Numeric)))
    puts val
  else
    case $currentToken.kind()
    when "id"
      if ($operator.kind() == "equals")
        name = $currentToken.value()
        assignment = expression()
        if (assignment != nil)
          $hashmap[name] = assignment
          puts "The variable #{name} has been assigned the value #{assignment}"
        else
          puts "Please enter a value to assign to the variable #{name}"
        end
      else
        puts "Identifier " + $currentToken.value() + " not assigned"
      end
    when "clear"
      id = $operator
      if (id.kind() == "id")
        $hashmap.delete(id.value())
        puts "The variable #{id.value()} has been deleted"
      else
        puts "Enter valid id to delete"
      end
    when "list"
      $hashmap.each do |ident, value|
        puts "#{ident}: #{value}"
      end
    when "eof"
      puts "End of File"
    when "quit", "exit"
      puts "Exiting"
      $running = false
    end
  end
end

def program(input)
  inputLines = input.split("\\n")
  # Each line should be a self contained statement
  inputLines.each do |line|
    semicolonsplit = line.split(";")
    semicolonsplit.each do |split|
      $scanner.setbuffer(split)
      # This is where the calculator does its thing
      statement()
    end
  end
end

while ($running)
  # Print preceding prompt
  print "Calculator> "
  # Get Input
  input = gets.chomp
  program(input)
end
