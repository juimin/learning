package com.example.lazydevs.lazydevsfinalproject;

import android.app.ProgressDialog;
import android.content.Intent;
import android.support.annotation.NonNull;
import android.support.v7.app.AppCompatActivity;
import android.os.Bundle;
import android.text.TextUtils;
import android.view.View;
import android.widget.Button;
import android.widget.EditText;
import android.widget.TextView;
import android.widget.Toast;

import com.google.android.gms.tasks.OnCompleteListener;
import com.google.android.gms.tasks.Task;
import com.google.firebase.auth.AuthResult;
import com.google.firebase.auth.FirebaseAuth;
import com.google.firebase.auth.FirebaseUser;
import com.google.firebase.database.DatabaseReference;
import com.google.firebase.database.FirebaseDatabase;

public class LoginActivity extends AppCompatActivity implements View.OnClickListener{
    private Button buttonSignin;
    private EditText editTextEmail;
    private EditText editTextPassword;
    private TextView textViewSignup;
    private ProgressDialog progressDialog;

    private FirebaseAuth firebaseAuth;
    private DatabaseReference databaseReference;


    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_login);

        firebaseAuth = firebaseAuth.getInstance();

        if(firebaseAuth.getCurrentUser() != null){
            // start searching activity here
            finish();
            startActivity(new Intent(getApplicationContext(), SearchingActivity.class));

        }

        databaseReference = FirebaseDatabase.getInstance().getReference();

        progressDialog =  new ProgressDialog(this);

        editTextEmail = (EditText) findViewById(R.id.editTextEmail);
        editTextPassword = (EditText) findViewById(R.id.editTextPassword);
        buttonSignin = (Button) findViewById(R.id.buttonSignin);
        textViewSignup = (TextView) findViewById(R.id.textViewSignup);

        buttonSignin.setOnClickListener(this);
        textViewSignup.setOnClickListener(this);
    }

    public void userLogin(){
        String email = editTextEmail.getText().toString().trim();
        String password = editTextPassword.getText().toString().trim();
        if(TextUtils.isEmpty(email)){
            //email is empty
            Toast.makeText(this, "Please enter email", Toast.LENGTH_SHORT).show();
            //stopping the function execution further
            return;
        }

        if(TextUtils.isEmpty(password)){
            //password is empty
            Toast.makeText(this, "Please enter password", Toast.LENGTH_SHORT).show();
            //stopping the function execution further
            return;
        }

        //if validation are good
        //we will first show a progress dialog
        progressDialog.setMessage("Logging In Please Wait...");
        progressDialog.show();

        firebaseAuth.signInWithEmailAndPassword(email, password)
                .addOnCompleteListener(this, new OnCompleteListener<AuthResult>() {
                    @Override
                    public void onComplete(@NonNull Task<AuthResult> task) {
                        progressDialog.dismiss();
                        if(task.isSuccessful()){
                            //save the correct user information in the database
                            saveUserInformation();
                            finish();
                            startActivity(new Intent(getApplicationContext(), SearchingActivity.class));
                        }else {
                            Toast.makeText(LoginActivity.this, "Could not Sign in, please try again", Toast.LENGTH_SHORT).show();
                        }
                    }
        });
    }

    private void saveUserInformation(){
        String email = editTextEmail.getText().toString().trim();

        UserInformation userInformation = new UserInformation(email);
        FirebaseUser user = firebaseAuth.getCurrentUser();

        databaseReference.child(user.getUid()).setValue(userInformation);

        Toast.makeText(this, "Information Saved", Toast.LENGTH_SHORT).show();
    }

    @Override
    public void onClick(View v) {
        if(v == buttonSignin){
            userLogin();
        }
        if(v == textViewSignup){
            finish();
            startActivity(new Intent(this, SignupActivity.class));
        }
    }
}
