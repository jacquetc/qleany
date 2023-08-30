#include "controller_registration.h"
#include "domain_registration.h"
#include "mainwindow.h"
#include "persistence_registration.h"
#include <QApplication>

int main(int argc, char *argv[])
{

    QApplication a(argc, argv);

    auto *domainRegistratoin = new Simple::Domain::DomainRegistration(&a);
    auto *persistenceRegistration = new Simple::Persistence::PersistenceRegistration(&a);
    auto *controllerRegistration =
        new Simple::Controller::ControllerRegistration(&a, persistenceRegistration->repositoryProvider());

    MainWindow w;
    w.show();
    return a.exec();
}
