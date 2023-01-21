/*
 Navicat Premium Data Transfer

 Source Server         : localhost_3306
 Source Server Type    : MySQL
 Source Server Version : 80030
 Source Host           : localhost:3306
 Source Schema         : oj

 Target Server Type    : MySQL
 Target Server Version : 80030
 File Encoding         : 65001

 Date: 20/01/2023 19:57:08
*/

SET NAMES utf8mb4;
SET FOREIGN_KEY_CHECKS = 0;

-- ----------------------------
-- Table structure for answer_info
-- ----------------------------
DROP TABLE IF EXISTS `answer_info`;
CREATE TABLE `answer_info`  (
  `answerId` int NOT NULL AUTO_INCREMENT,
  `contestId` int NOT NULL,
  `questionId` int NOT NULL,
  `username` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL,
  `answerTime` varchar(255) NULL DEFAULT NULL,
  `answerContent` varchar(15000) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL,
  `result` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL,
  `runTime` float NULL DEFAULT NULL,
  PRIMARY KEY (`answerId`) USING BTREE,
  INDEX `contestId`(`contestId` ASC) USING BTREE,
  INDEX `questionId`(`questionId` ASC) USING BTREE,
  INDEX `username`(`username` ASC) USING BTREE,
  CONSTRAINT `answer_info_ibfk_2` FOREIGN KEY (`questionId`) REFERENCES `question_info` (`questionId`) ON DELETE RESTRICT ON UPDATE RESTRICT,
  CONSTRAINT `answer_info_ibfk_3` FOREIGN KEY (`username`) REFERENCES `user_info` (`username`) ON DELETE RESTRICT ON UPDATE RESTRICT
) ENGINE = InnoDB AUTO_INCREMENT = 2 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci ROW_FORMAT = DYNAMIC;

-- ----------------------------
-- Records of answer_info
-- ----------------------------
INSERT INTO `answer_info` VALUES (1, 1, 2, 'aa12', '2023-01-27 20:29:34', 'abc', 'Answer Correct', 220);

-- ----------------------------
-- Table structure for contest_info
-- ----------------------------
DROP TABLE IF EXISTS `contest_info`;
CREATE TABLE `contest_info`  (
  `contestId` int NOT NULL AUTO_INCREMENT,
  `contestTitle` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL,
  `username` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL,
  `startTime` datetime NULL DEFAULT NULL,
  `endTime` datetime NULL DEFAULT NULL,
  PRIMARY KEY (`contestId`) USING BTREE,
  INDEX `username`(`username` ASC) USING BTREE,
  CONSTRAINT `contest_info_ibfk_1` FOREIGN KEY (`username`) REFERENCES `user_info` (`username`) ON DELETE RESTRICT ON UPDATE RESTRICT
) ENGINE = InnoDB AUTO_INCREMENT = 905 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci ROW_FORMAT = DYNAMIC;

-- ----------------------------
-- Records of contest_info
-- ----------------------------
INSERT INTO `contest_info` VALUES (1, 'test', '111', '2022-12-28 20:25:40', '2023-01-12 20:25:42');
INSERT INTO `contest_info` VALUES (2, 'abc', '111', '2022-12-29 20:25:54', '2023-01-08 20:25:58');
INSERT INTO `contest_info` VALUES (3, 'test24', '111', '2002-02-01 00:00:00', '2002-02-19 00:00:00');
INSERT INTO `contest_info` VALUES (4, '67567', '111', '2002-02-01 00:00:00', '2002-02-19 00:00:00');
INSERT INTO `contest_info` VALUES (5, 'hjvh', '111', '2023-01-03 00:00:00', '2023-01-18 00:00:00');
INSERT INTO `contest_info` VALUES (6, 'sadsad', '111', '2023-01-03 00:00:00', '2023-01-13 00:00:00');
INSERT INTO `contest_info` VALUES (7, 'i am a contest', '111', '2023-01-01 00:00:00', '2023-01-31 00:00:00');

-- ----------------------------
-- Table structure for contest_question
-- ----------------------------
DROP TABLE IF EXISTS `contest_question`;
CREATE TABLE `contest_question`  (
  `contestId` int NOT NULL,
  `questionId` int NOT NULL,
  INDEX `contestId`(`contestId` ASC) USING BTREE,
  INDEX `questionId`(`questionId` ASC) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci ROW_FORMAT = DYNAMIC;

-- ----------------------------
-- Records of contest_question
-- ----------------------------
INSERT INTO `contest_question` VALUES (1, 2);
INSERT INTO `contest_question` VALUES (2, 1);
INSERT INTO `contest_question` VALUES (2, 4);
INSERT INTO `contest_question` VALUES (5, 1);
INSERT INTO `contest_question` VALUES (5, 2);
INSERT INTO `contest_question` VALUES (5, 3);
INSERT INTO `contest_question` VALUES (6, 1);
INSERT INTO `contest_question` VALUES (6, 2);
INSERT INTO `contest_question` VALUES (6, 4);
INSERT INTO `contest_question` VALUES (7, 3);
INSERT INTO `contest_question` VALUES (7, 4);
INSERT INTO `contest_question` VALUES (7, 5);

-- ----------------------------
-- Table structure for question_info
-- ----------------------------
DROP TABLE IF EXISTS `question_info`;
CREATE TABLE `question_info`  (
  `questionId` int NOT NULL,
  `questionTitle` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL,
  `questionContent` varchar(1000) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL,
  `questionExample` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL,
  PRIMARY KEY (`questionId`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci ROW_FORMAT = DYNAMIC;

-- ----------------------------
-- Records of question_info
-- ----------------------------
INSERT INTO `question_info` VALUES (1, 'q1', 'afdsaf', '2321');
INSERT INTO `question_info` VALUES (2, 'q2', 'agdvafas', '234234');
INSERT INTO `question_info` VALUES (3, 'q3', 'dsgdfgasfd', '43342');
INSERT INTO `question_info` VALUES (4, 'q4', 'ewrfsad', '43523');
INSERT INTO `question_info` VALUES (5, 'q5', 'dfdsfdsfds', 'dsfsdfsd');

-- ----------------------------
-- Table structure for user_info
-- ----------------------------
DROP TABLE IF EXISTS `user_info`;
CREATE TABLE `user_info`  (
  `username` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL,
  `password` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL,
  `isManager` int NULL DEFAULT NULL,
  `acnums` int NULL DEFAULT NULL,
  PRIMARY KEY (`username`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci ROW_FORMAT = DYNAMIC;

-- ----------------------------
-- Records of user_info
-- ----------------------------
INSERT INTO `user_info` VALUES ('111', '222222', 1, 1);
INSERT INTO `user_info` VALUES ('aa12', 'bb3456', 0, 2);
INSERT INTO `user_info` VALUES ('qwe', 'asdefg', 0, 2);
INSERT INTO `user_info` VALUES ('test100', '123456', 0, 3);
INSERT INTO `user_info` VALUES ('test101', '123456', 0, 0);

SET FOREIGN_KEY_CHECKS = 1;
