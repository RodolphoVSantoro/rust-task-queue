#include <iostream>
#include <stdlib.h>
#include <string>
#include <vector>
using namespace std;

vector<string> split (string s, string delimiter) {
    size_t pos_start = 0, pos_end, delim_len = delimiter.length();
    string token;
    vector<string> res;

    while ((pos_end = s.find (delimiter, pos_start)) != string::npos) {
        token = s.substr (pos_start, pos_end - pos_start);
        pos_start = pos_end + delim_len;
        res.push_back (token);
    }

    res.push_back (s.substr (pos_start));
    return res;
}

int main(int argc, char** argv){
  	int cont = 0;
    int mult = 1;
    for (int i = 1; i < argc; ++i){
        vector<string> v = split(argv[i], ":");
        int n = 0;
        if(v.size()>1)
            n = stoi(v[1]);
        cont+=n;
        mult*=n;
    }
    cout << "soma:" << cont << endl;
    cout << "mult:" << mult << endl;
    return 0;
}
