//
//  main.m
//  cargo-lipo-test
//
//  Created by Tim Neumann on 01.01.19.
//  Copyright © 2019 Tim Neumann. All rights reserved.
//

#import <UIKit/UIKit.h>
#import "AppDelegate.h"

void hello_from_static1(void);
void hello_from_static2(void);
void hello_from_static3(void);

int main(int argc, char * argv[]) {
    hello_from_static1();
    hello_from_static2();
    hello_from_static3();

    @autoreleasepool {
        return UIApplicationMain(argc, argv, nil, NSStringFromClass([AppDelegate class]));
    }
}
