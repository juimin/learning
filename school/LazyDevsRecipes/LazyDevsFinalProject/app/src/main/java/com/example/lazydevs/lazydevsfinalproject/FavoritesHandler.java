package com.example.lazydevs.lazydevsfinalproject;

import java.util.ArrayList;

import com.google.firebase.auth.FirebaseAuth;
import com.google.firebase.auth.FirebaseUser;
import com.google.firebase.database.DataSnapshot;
import com.google.firebase.database.DatabaseError;
import com.google.firebase.database.DatabaseReference;
import com.google.firebase.database.FirebaseDatabase;
import com.google.firebase.database.GenericTypeIndicator;
import com.google.firebase.database.ValueEventListener;

public class FavoritesHandler {

    private FirebaseAuth firebaseAuth;
    private FirebaseUser user;
    private DatabaseReference userFavoritesReference;
    private ArrayList<FavoriteRecipe> currentFavorites;

    public FavoritesHandler() {
        firebaseAuth = firebaseAuth.getInstance();
        user = firebaseAuth.getCurrentUser();
        userFavoritesReference = FirebaseDatabase.getInstance().getReference().child("favorites");
    }

    //adds a new FavoriteRecipe object to the current full favorites ArrayList
    public void addFavorite(FavoriteRecipe newFavorite) {
        currentFavorites = getFavorites();
        currentFavorites.add(newFavorite);
        userFavoritesReference.setValue(currentFavorites);
    }

    //returns the JSONArray of user's favorite recipes
    public ArrayList<FavoriteRecipe> getFavorites() {
        userFavoritesReference.addListenerForSingleValueEvent(new ValueEventListener() {
            @Override
            public void onDataChange(DataSnapshot dataSnapshot) {
                GenericTypeIndicator<ArrayList<FavoriteRecipe>> t = new
                        GenericTypeIndicator<ArrayList<FavoriteRecipe>>() {};
                currentFavorites = dataSnapshot.getValue(t);
            }

            @Override
            public void onCancelled(DatabaseError databaseError) {

            }
        });
        return currentFavorites;
    }
}
