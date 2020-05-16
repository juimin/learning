#include <iostream>

int add(int x, int y) {
   return x + y;
}

int main() {
    std::cout << "Lets try to do some addition\n";
    std::cout << "Please Enter Two Numbers:\n";
    int a, b;
    std::cin >> a;
    std::cin >> b;
    std::cout << "The sum of " << a << " and " << b << " is " << add(a,b) << "\n";
    return 0;
}