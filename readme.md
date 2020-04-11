# MPU6050 accelerometer - DRAFT

dev board: STM32F051

Currently just prints raw readings to the console with semihosting.

https://docs.rs/mpu6050/0.1.3/mpu6050/ - crate description

https://invensense.tdk.com/wp-content/uploads/2015/02/MPU-6500-Datasheet2.pdf - sensor datasheet

TO DO:

* log the readings to console or SD card
* connect a display and use the readings to move a point or ideally implement this: 
https://www.twobitarcade.net/article/gyroscopic-wireframe-cube/
