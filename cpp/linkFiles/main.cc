#include <iostream>

#include "lib/subtract.h"

int add(int x, int y) {
   return x + y;
}

int main() {
    std::cout << "Lets try to do some math\n";
    std::cout << "Please Enter Two Numbers:\n";
    int a, b;
    std::cin >> a;
    std::cin >> b;
    std::cout << "The sum of " << a << " and " << b << " is " << add(a,b) << "\n";
    std::cout << "The difference between " << a << " and " << b << " is " << subtract(a,b) << "\n";
    return 0;
}