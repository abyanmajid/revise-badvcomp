#include <iostream>
#include <complex>
#include <cstdlib>
#include <ctime>
#include <sstream>

std::string format_double(double value) {
    std::ostringstream out;
    if (value == static_cast<long>(value))
        out << static_cast<long>(value);
    else
        out << value;
    std::string s = out.str();
    size_t dot = s.find('.');
    if (dot != std::string::npos) {
        s.erase(s.find_last_not_of('0') + 1);
        if (s.back() == '.')
            s.pop_back();
    }
    return s;
}

std::string format_complex(const std::complex<double>& c) {
    std::string realPart = format_double(c.real()), imagPart = format_double(c.imag());
    if (c.real() == 0 && c.imag() == 0)
        return "0";
    else if (c.imag() == 0)
        return realPart;
    else if (c.real() == 0)
        return imagPart + "i";
    else
        return realPart + (c.imag() > 0 ? "+" : "") + imagPart + "i";
}

std::complex<double> generate_random_complex() {
    double realPart = (rand() % 41) - 20;
    double imagPart = (rand() % 41) - 20;
    return std::complex<double>(realPart, imagPart);
}

std::pair<std::string, std::string> operate_complex(char op) {
    std::complex<double> z = generate_random_complex(), w = generate_random_complex();
    std::complex<double> result;
    switch (op) {
        case '+': result = z + w; break;
        case '-': result = z - w; break;
        case '*': result = z * w; break;
        case '/': result = w != std::complex<double>(0, 0) ? z / w : std::complex<double>(0, 0); break;
    }
    std::string question = "What is (" + format_complex(z) + ") " + op + " (" + format_complex(w) + ")?";
    std::string answer = format_complex(result);
    return {question, answer};
}
