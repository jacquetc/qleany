#include "interactor_registration.h"
#include "domain_registration.h"
#include "mainwindow.h"
#include "persistence_registration.h"
#include <QApplication>

int main(int argc, char *argv[])
{

    QApplication a(argc, argv);

    new Simple::Domain::DomainRegistration(&a);
    auto *persistenceRegistration = new Simple::Persistence::PersistenceRegistration(&a);
    new Simple::Interactor::InteractorRegistration(&a, persistenceRegistration->repositoryProvider());

    MainWindow w;
    w.show();
    w.init();

    return a.exec();
}
