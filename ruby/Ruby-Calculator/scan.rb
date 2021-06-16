# Derek Wang
# CSE 413 16AU
# HW 7 - Regular Expressions and Scanners

# Part II. Calculator Scanner

# This file contains the Token class and the Scanner

# readline provides nice command-line editing for text input
require 'readline'

class Scanner
    attr_accessor :nextch, :input, :index
    # Set the current looked at string to be the buffered line
    def setbuffer(input)
      @input = input
      @index = 0
      @nextch = input[@index]
    end

    # Figure out and return the next token
    def next_token()
      skipwhitespace()
      if (/[\+\-()\/\*=#\\;]/.match(@nextch))
        case @nextch
        when "("
          symbolToken = Token.new(Token::LPAREN, "Left parenthese")
        when ")"
          symbolToken = Token.new(Token::RPAREN, "Right parenthese")
        when "+"
          symbolToken = Token.new(Token::ADD, "Addition symbol")
        when "-"
          symbolToken = Token.new(Token::SUBTRACT, "Subtraction symbol")
        when "/"
          symbolToken = Token.new(Token::DIVIDE, "Division symbol")
        when "\\"
          if (@input[@index + 1] == "n")
            symbolToken = Token.new(Token::NEWLINE, "newline symbol")
            @index += 1
          else
            puts "Error. Unsupported character \\" + @input[@index + 1]
            symbolToken = nil
          end
        when "*"
          if (@input[@index + 1] == "*")
            symbolToken = Token.new(Token::POWER, "Power symbol")
            @index += 1
          else
            symbolToken = Token.new(Token::MULTIPLY, "Multiplication symbol")
          end
        when "="
          symbolToken = Token.new(Token::EQUALS, "Equal symbol")
        when "#"
          symbolToken = Token.new(Token::COMMENT, "Comment Symbol")
        end
        getch()
        return symbolToken
      elsif (/[a-zA-Z]/.match(@nextch))
        output = @nextch
        getch()
        while (/[a-zA-Z0-9_]/.match(@nextch))
          output = output + @nextch
          getch()
        end
        case output.downcase
        when "list"
          return Token.new(Token::LIST, "List all ids command")
        when "clear"
          return Token.new(Token::CLEAR, "Clear command for specified id")
        when "quit"
          return Token.new(Token::QUIT, "Quit command")
        when "exit"
          return Token.new(Token::EXIT, "Exit command")
        when "sqrt"
          return Token.new(Token::SQRT, "Square root command")
        when "log"
          return Token.new(Token::LOG, "Log base e command")
        when "cos"
          return Token.new(Token::COS, "Cosine command")
        when "sin"
          return Token.new(Token::SIN, "Sine command")
        when "tan"
          return Token.new(Token::TAN, "Tangent command")
        else
          return Token.new(Token::ID, "Identifier: " + output, output)
        end
      elsif (/[0-9\.]/.match(@nextch))
        output = @nextch
        getch()
        regex = /[0-9e\.]/
        while (regex.match(@nextch))
          if (/[e]/.match(@nextch))
            output = output + @nextch
            if (/[-]/.match(@input[@index + 1]))
              getch()
              output = output + "-"
            end
          else
            output = output + @nextch
          end
          getch()
        end
        if (/[-]?[0-9]*\.?[0-9]+([eE][-+]?[0-9]+)?/.match(output))
          symbolToken = Token.new(Token::NUMBER, "Number: " + output, output)
        else
          puts "Error: invalid number input"
          return nil
        end
      elsif (@nextch == nil)
        return Token.new(Token::EOF, "End of File symbol")
      else
        # Invalid token
        puts "Error: Invalid Token " + @nextch
        return nil
      end
    end

    # Get the next character
    def getch()
      @index += 1
      @nextch = input[@index]
    end

    # Skip the white space till the nextr character
    def skipwhitespace()
      while(/[^\S\x0a\x0d]/.match(@nextch))
        getch()
      end
    end
end

# This is the token class that keeps track of the value and the token's type
# can display the value of the token, the kind of token, and output the token to String
class Token
    attr_reader :kind, :value, :desc
    ADD ||= "add"
    EOF ||= "eof"
    EQUALS ||= "equals"
    SUBTRACT ||= "subtract"
    MULTIPLY ||= "multiply"
    DIVIDE ||= "divide"
    NUMBER ||= "num"
    ID ||= "id"
    LPAREN ||= "lparen"
    RPAREN ||= "rparen"
    EXIT ||= "exit"
    QUIT ||= "quit"
    POWER ||= "power"
    CLEAR ||= "clear"
    LIST ||= "list"
    SQRT ||= "sqrt"
    COMMENT ||= "comment"
    NEWLINE ||= "newline"
    LOG ||= "log"
    SIN ||= "sin"
    COS ||= "cos"
    TAN ||= "tan"

    # Init the token with the desired kind, description, and value
    def initialize(kind, desc, value = nil)
        @kind = kind
        @desc = desc
        @value = value
    end
    # Returns the kind of token this is as a string
    def kind()
      return @kind
    end
    # Return the value of this token based on the kind that it is
    def value()
      case @kind
      when "id", "num"
        return @value
      else
        # Value undefined for classes not of identifiers or numbers
        return nil
      end
    end
    # Standard ruby to String Methods
    def to_s()
      return @desc
    end
end
