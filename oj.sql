/*
 Navicat Premium Data Transfer

 Source Server         : localhost
 Source Server Type    : MySQL
 Source Server Version : 80018
 Source Host           : localhost:3306
 Source Schema         : oj

 Target Server Type    : MySQL
 Target Server Version : 80018
 File Encoding         : 65001

 Date: 21/01/2023 18:47:18
*/

SET NAMES utf8mb4;
SET FOREIGN_KEY_CHECKS = 0;

-- ----------------------------
-- Table structure for answer_info
-- ----------------------------
DROP TABLE IF EXISTS `answer_info`;
CREATE TABLE `answer_info`  (
  `answerId` int(11) NOT NULL AUTO_INCREMENT,
  `contestId` int(11) NOT NULL,
  `questionId` int(11) NOT NULL,
  `username` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL,
  `answerTime` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL,
  `answerContent` varchar(15000) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL,
  `result` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL,
  `runTime` float NULL DEFAULT NULL,
  PRIMARY KEY (`answerId`) USING BTREE,
  INDEX `contestId`(`contestId`) USING BTREE,
  INDEX `questionId`(`questionId`) USING BTREE,
  INDEX `username`(`username`) USING BTREE,
  CONSTRAINT `answer_info_ibfk_2` FOREIGN KEY (`questionId`) REFERENCES `question_info` (`questionId`) ON DELETE RESTRICT ON UPDATE RESTRICT,
  CONSTRAINT `answer_info_ibfk_3` FOREIGN KEY (`username`) REFERENCES `user_info` (`username`) ON DELETE RESTRICT ON UPDATE RESTRICT
) ENGINE = InnoDB AUTO_INCREMENT = 2 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci ROW_FORMAT = DYNAMIC;

-- ----------------------------
-- Records of answer_info
-- ----------------------------
INSERT INTO `answer_info` VALUES (1, 1, 2, 'aa12', '2023-01-21 20:29:34', 'abc', 'Answer Correct', 220);
INSERT INTO `answer_info` VALUES (2, 1, 2, 'aa12', '2023-01-27 22:29:34', 'test', 'Compilation Error', 20);
INSERT INTO `answer_info` VALUES (3, 2, 1, 'qwe', '2023-01-27 22:29:34', 'adf', 'Time Limit', 500);

-- ----------------------------
-- Table structure for contest_info
-- ----------------------------
DROP TABLE IF EXISTS `contest_info`;
CREATE TABLE `contest_info`  (
  `contestId` int(11) NOT NULL AUTO_INCREMENT,
  `contestTitle` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL,
  `username` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL,
  `startTime` datetime NULL DEFAULT NULL,
  `endTime` datetime NULL DEFAULT NULL,
  PRIMARY KEY (`contestId`) USING BTREE,
  INDEX `username`(`username`) USING BTREE,
  CONSTRAINT `contest_info_ibfk_1` FOREIGN KEY (`username`) REFERENCES `user_info` (`username`) ON DELETE RESTRICT ON UPDATE RESTRICT
) ENGINE = InnoDB AUTO_INCREMENT = 905 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci ROW_FORMAT = DYNAMIC;

-- ----------------------------
-- Records of contest_info
-- ----------------------------
INSERT INTO `contest_info` VALUES (1, ' 寒假训练赛（一)', '111', '2023-01-05 20:25:40', '2023-02-28 20:25:42');
INSERT INTO `contest_info` VALUES (2, 'Latihan Graph', '李四', '2022-12-29 20:25:54', '2023-01-08 20:25:58');
INSERT INTO `contest_info` VALUES (3, '	\r\nDA CPC contest 01-20', '张三', '2023-02-01 00:00:00', '2023-02-18 00:00:00');
INSERT INTO `contest_info` VALUES (4, 'stress/fight/improvement 01', '111', '2023-01-01 00:00:00', '2023-12-11 00:00:00');

-- ----------------------------
-- Table structure for contest_question
-- ----------------------------
DROP TABLE IF EXISTS `contest_question`;
CREATE TABLE `contest_question`  (
  `contestId` int(11) NOT NULL,
  `questionId` int(11) NOT NULL,
  INDEX `contestId`(`contestId`) USING BTREE,
  INDEX `questionId`(`questionId`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci ROW_FORMAT = DYNAMIC;

-- ----------------------------
-- Records of contest_question
-- ----------------------------
INSERT INTO `contest_question` VALUES (1, 1);
INSERT INTO `contest_question` VALUES (2, 1);
INSERT INTO `contest_question` VALUES (2, 4);
INSERT INTO `contest_question` VALUES (3, 1);
INSERT INTO `contest_question` VALUES (3, 3);
INSERT INTO `contest_question` VALUES (4, 1);
INSERT INTO `contest_question` VALUES (4, 2);
INSERT INTO `contest_question` VALUES (4, 3);
INSERT INTO `contest_question` VALUES (4, 4);
INSERT INTO `contest_question` VALUES (1, 2);

-- ----------------------------
-- Table structure for question_info
-- ----------------------------
DROP TABLE IF EXISTS `question_info`;
CREATE TABLE `question_info`  (
  `questionId` int(11) NOT NULL,
  `questionTitle` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL,
  `questionContent` varchar(1000) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL,
  `questionExample` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL,
  PRIMARY KEY (`questionId`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci ROW_FORMAT = DYNAMIC;

-- ----------------------------
-- Records of question_info
-- ----------------------------
INSERT INTO `question_info` VALUES (1, 'A+B Problem', '输入两个整数 \r\n\r\n\r\n\r\na,b，输出它们的和', 'Input:20 30                                  Output:50');
INSERT INTO `question_info` VALUES (2, '小玉在游泳', '小玉开心的在游泳，可是她很快难过的发现，自己的力气不够，游泳好累哦。已知小玉第一步能游 \r\n2\r\n2 米，可是随着越来越累，力气越来越小，她接下来的每一步都只能游出上一步距离的 \r\n98\r\n%。现在小玉想知道，如果要游到距离 \r\n\r\nx 米的地方，她需要游多少步呢。请你编程解决这个问题。', 'Input：4 3             Output: 3');
INSERT INTO `question_info` VALUES (3, '计算阶乘', '求 n!，也就是 1×2×3⋯×n。', 'Input:3    Output:6');
INSERT INTO `question_info` VALUES (4, 'q4', 'test', 'Intput:123 Output:31');
INSERT INTO `question_info` VALUES (5, 'q5', 'dfdsfdsfds', 'dsfsdfsd');

-- ----------------------------
-- Table structure for user_info
-- ----------------------------
DROP TABLE IF EXISTS `user_info`;
CREATE TABLE `user_info`  (
  `username` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL,
  `password` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL,
  `isManager` int(11) NULL DEFAULT NULL,
  `acnums` int(11) NULL DEFAULT NULL,
  PRIMARY KEY (`username`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci ROW_FORMAT = DYNAMIC;

-- ----------------------------
-- Records of user_info
-- ----------------------------
INSERT INTO `user_info` VALUES ('111', '222222', 1, 1);
INSERT INTO `user_info` VALUES ('aa12', 'bb3456', 0, 2);
INSERT INTO `user_info` VALUES ('qwe', 'asdefg', 0, 2);
INSERT INTO `user_info` VALUES ('张三', '123456', 1, 3);
INSERT INTO `user_info` VALUES ('李四', '123456', 1, 0);

SET FOREIGN_KEY_CHECKS = 1;
