using GUIs.Common;
using System;
using System.Collections.Generic;
using Windows.UI.Xaml.Controls;

// The User Control item template is documented at https://go.microsoft.com/fwlink/?LinkId=234236

namespace GUIs.Views
{
    public sealed partial class Cells : UserControl
    {
        public Row[] GridData = new Row[100];

        public Cells()
        {
            InitializeComponent();
            for (int i = 0; i < 100; i++)
            {
                GridData[i] = new Row(i);
            }
        }
    }

    public class Row
    {
        private int index;
        private Signal<Expression>[] cells = new Signal<Expression>[26];

        public int Index { get => index; set => index = value; }
        public Expression A => cells[0].Value;
        public Expression B => cells[1].Value;
        public Expression C => cells[2].Value;
        public Expression D => cells[3].Value;
        public Expression E => cells[4].Value;
        public Expression F => cells[5].Value;
        public Expression G => cells[6].Value;
        public Expression H => cells[7].Value;
        public Expression I => cells[8].Value;
        public Expression J => cells[9].Value;
        public Expression K => cells[10].Value;
        public Expression L => cells[11].Value;
        public Expression M => cells[12].Value;
        public Expression N => cells[13].Value;
        public Expression O => cells[14].Value;
        public Expression P => cells[15].Value;
        public Expression Q => cells[16].Value;
        public Expression R => cells[17].Value;
        public Expression S => cells[18].Value;
        public Expression T => cells[19].Value;
        public Expression U => cells[20].Value;
        public Expression V => cells[21].Value;
        public Expression W => cells[22].Value;
        public Expression X => cells[23].Value;
        public Expression Y => cells[24].Value;
        public Expression Z => cells[25].Value;

        public Row(int index)
        {
            Index = index;
            for (int i = 0; i < 26; i++)
            {
                cells[i] = new Signal<Expression>(new Expression(ExpressionOperator.Addition, null, null, 42));
            }
        }
    }

    public class Cell
    {

    }

    public class Expression : Trackable
    {
        private ExpressionOperator _operator;
        private Expression firstOperand;
        private Expression secondOperand;
        private double result;

        public Expression(ExpressionOperator _operator, Expression firstOperand, Expression secondOperand, double result)
        {
            this._operator = _operator;
            this.firstOperand = firstOperand;
            this.secondOperand = secondOperand;
            this.result = result;
        }

        public ExpressionOperator Operator
        {
            get => _operator;
            set
            {
                if (Operator != value)
                {
                    _operator = value;
                    Evaluate();
                    Track(nameof(Operator));
                }

            }
        }
        public Expression FirstOperand
        {
            get => firstOperand;
            set
            {
                if (FirstOperand != value)
                {
                    firstOperand = value;
                    Evaluate();
                    Track(nameof(FirstOperand));
                }

            }
        }
        public Expression SecondOperand
        {
            get => secondOperand;
            set
            {
                if (SecondOperand != value)
                {
                    secondOperand = value;
                    Evaluate();
                    Track(nameof(SecondOperand));
                }

            }
        }
        public double Result
        {
            get => result;
            set
            {
                if (Result != value)
                {
                    result = value;
                    Track(nameof(Result));
                }
            }
        }

        public void Evaluate()
        {
            switch (Operator)
            {
                case ExpressionOperator.Assignment:
                    Result = FirstOperand.Result;
                    break;
                case ExpressionOperator.Addition:
                    Result = FirstOperand.Result + SecondOperand.Result;
                    break;
                case ExpressionOperator.Multiplication:
                    Result = FirstOperand.Result * SecondOperand.Result;
                    break;
                case ExpressionOperator.Subtraction:
                    Result = FirstOperand.Result - SecondOperand.Result;
                    break;
                case ExpressionOperator.Division:
                    Result = FirstOperand.Result / SecondOperand.Result;
                    break;
                default:
                    break;
            }
        }
    }

    public enum ExpressionOperator
    {
        Assignment,
        Addition,
        Multiplication,
        Subtraction,
        Division
    }
}
