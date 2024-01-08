'''
######################################################

				Welcome to coder club

		A fully self made repo by Rahul Surana

You can use any of these files and twig them to your use

	have fun and also star mark this repo more to come

########################################################
'''



import os

f = open("README.md", "w")

f.write("## Hi Guys \n\n# *Welcome To My Repository*\n")

f.write("### <div style='text-align:right'><sub> - Rahul Surana</sub></div>\n")
f.write("### So Heres how the Journey is Going We have Gone Through Following Modulues : \n")

for r,d,x in list(os.walk(os.getcwd())):
	# print(f)
	for files in x:	
		if files[-3:] == ".rs":
			f.write("- "+ files[:-3] +"\n")
			print(files)
f.write("\n")

f.write("### Please Do Star the Repo if it ever helps you. Also Would Like to Form a Community So we all can grow Together\n")

f.close()
